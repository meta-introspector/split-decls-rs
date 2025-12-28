macro_rules! deps {
    () => {
        Parser!();
        Marker!();
        Event!();
        CompletedMarker!();
    };
}

macro_rules! impl_72 {
    () => {
        deps!();
        impl Marker { fn new (pos : u32) -> Marker { Marker { pos , bomb : DropBomb :: new ("Marker must be either completed or abandoned") } } # [doc = " Finishes the syntax tree node and assigns `kind` to it,"] # [doc = " and mark the create a `CompletedMarker` for possible future"] # [doc = " operation like `.precede()` to deal with forward_parent."] pub (crate) fn complete (mut self , p : & mut Parser < '_ > , kind : SyntaxKind) -> CompletedMarker { self . bomb . defuse () ; let idx = self . pos as usize ; match & mut p . events [idx] { Event :: Start { kind : slot , .. } => { * slot = kind ; } _ => unreachable ! () , } p . push_event (Event :: Finish) ; let end_pos = p . events . len () as u32 ; CompletedMarker :: new (self . pos , end_pos , kind) } # [doc = " Abandons the syntax tree node. All its children"] # [doc = " are attached to its parent instead."] pub (crate) fn abandon (mut self , p : & mut Parser < '_ >) { self . bomb . defuse () ; let idx = self . pos as usize ; if idx == p . events . len () - 1 { assert ! (matches ! (p . events . pop () , Some (Event :: Start { kind : TOMBSTONE , forward_parent : None }))) ; } } }
    };
}

impl_72!();