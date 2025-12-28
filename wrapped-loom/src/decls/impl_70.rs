macro_rules! deps {
    () => {
        Synchronize!();
        Notify!();
        State!();
    };
}

macro_rules! impl_70 {
    () => {
        deps!();
        impl Notify { pub (crate) fn new (seq_cst : bool , spurious : bool) -> Notify { super :: execution (| execution | { let state = execution . objects . insert (State { spurious , did_spur : false , seq_cst , notified : false , last_access : None , synchronize : Synchronize :: new () , }) ; trace ! (? state , ? seq_cst , ? spurious , "Notify::new") ; Notify { state } }) } pub (crate) fn notify (self , location : Location) { self . state . branch_opaque (location) ; rt :: execution (| execution | { let state = self . state . get_mut (& mut execution . objects) ; state . synchronize . sync_store (& mut execution . threads , Release) ; if state . seq_cst { execution . threads . seq_cst () ; } state . notified = true ; let (active , inactive) = execution . threads . split_active () ; for thread in inactive { let obj = thread . operation . as_ref () . map (| operation | operation . object ()) ; if obj == Some (self . state . erase ()) { trace ! (state = ? self . state , thread = ? thread . id , "Notify::notify") ; thread . unpark (active) ; } } }) ; } pub (crate) fn wait (self , location : Location) { let (notified , spurious) = rt :: execution (| execution | { let spurious = if self . state . get (& execution . objects) . might_spur () { execution . path . branch_spurious () } else { false } ; let state = self . state . get_mut (& mut execution . objects) ; if spurious { state . did_spur = true ; } trace ! (state = ? self . state , notified = ? state . notified , ? spurious , "Notify::wait 1") ; dbg ! ((state . notified , spurious)) }) ; if spurious { rt :: yield_now () ; return ; } if notified { self . state . branch_opaque (location) ; } else { self . state . branch_acquire (true , location) } super :: execution (| execution | { trace ! (state = ? self . state , "Notify::wait 2") ; let state = self . state . get_mut (& mut execution . objects) ; assert ! (state . notified) ; state . synchronize . sync_load (& mut execution . threads , Acquire) ; if state . seq_cst { execution . threads . seq_cst () ; } state . notified = false ; }) ; } }
    };
}

impl_70!();