macro_rules! deps {
    () => {
        OnDuplicate!();
        Stage!();
        SingleAttributeParser!();
        AcceptContext!();
        UnusedMultiple!();
    };
}

macro_rules! impl_248 {
    () => {
        deps!();
        impl < S : Stage > OnDuplicate < S > { fn exec < P : SingleAttributeParser < S > > (& self , cx : & mut AcceptContext < '_ , '_ , S > , used : Span , unused : Span ,) { match self { OnDuplicate :: Warn => cx . warn_unused_duplicate (used , unused) , OnDuplicate :: WarnButFutureError => cx . warn_unused_duplicate_future_error (used , unused) , OnDuplicate :: Error => { cx . emit_err (UnusedMultiple { this : used , other : unused , name : Symbol :: intern (& P :: PATH . into_iter () . map (| i | i . to_string ()) . collect :: < Vec < _ > > () . join ("..") ,) , }) ; } OnDuplicate :: Ignore => { } OnDuplicate :: Custom (f) => f (cx , used , unused) , } } }
    };
}

impl_248!();