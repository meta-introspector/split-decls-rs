macro_rules! deps {
    () => {
        Target!();
        AttrPath!();
    };
}

macro_rules! AttributeLintKind {
    () => {
        deps!();
        # [derive (Clone , Debug , HashStable_Generic)] pub enum AttributeLintKind { UnusedDuplicate { this : Span , other : Span , warning : bool } , IllFormedAttributeInput { suggestions : Vec < String > } , EmptyAttribute { first_span : Span } , InvalidTarget { name : AttrPath , target : Target , applied : Vec < String > , only : & 'static str } , InvalidStyle { name : AttrPath , is_used_as_inner : bool , target : Target , target_span : Span } , }
    };
}

AttributeLintKind!();