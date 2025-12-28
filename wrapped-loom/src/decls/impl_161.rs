macro_rules! deps {
    () => {
        VersionVec!();
        Thread!();
        State!();
        Id!();
    };
}

macro_rules! impl_161 {
    () => {
        deps!();
        impl Thread { fn new (id : Id , parent_span : & tracing :: Span) -> Thread { Thread { id , span : tracing :: info_span ! (parent : parent_span . id () , "thread" , id = id . id) , state : State :: Runnable { unparked : false } , critical : false , operation : None , causality : VersionVec :: new () , released : VersionVec :: new () , dpor_vv : VersionVec :: new () , last_yield : None , yield_count : 0 , locals : HashMap :: new () , } } pub (crate) fn is_runnable (& self) -> bool { matches ! (self . state , State :: Runnable { .. }) } pub (crate) fn set_runnable (& mut self) { self . state = State :: Runnable { unparked : false } ; } pub (crate) fn set_blocked (& mut self , location : Location) { self . state = State :: Blocked (location) ; } pub (crate) fn is_blocked (& self) -> bool { matches ! (self . state , State :: Blocked (..)) } pub (crate) fn is_yield (& self) -> bool { matches ! (self . state , State :: Yield) } pub (crate) fn set_yield (& mut self) { self . state = State :: Yield ; self . last_yield = Some (self . causality [self . id]) ; self . yield_count += 1 ; } pub (crate) fn is_terminated (& self) -> bool { matches ! (self . state , State :: Terminated) } pub (crate) fn set_terminated (& mut self) { self . state = State :: Terminated ; } pub (crate) fn drop_locals (& mut self) -> Box < dyn std :: any :: Any > { let mut locals = Vec :: with_capacity (self . locals . len ()) ; for local in self . locals . values_mut () { locals . push (local . 0 . take ()) ; } Box :: new (locals) } pub (crate) fn unpark (& mut self , unparker : & Thread) { self . causality . join (& unparker . causality) ; self . set_unparked () ; } # [doc = " Unpark a thread's state. If it is already runnable, store the unpark for"] # [doc = " a future call to `park`."] fn set_unparked (& mut self) { if self . is_blocked () || self . is_yield () { self . set_runnable () ; } else if self . is_runnable () { self . state = State :: Runnable { unparked : true } } } }
    };
}

impl_161!()