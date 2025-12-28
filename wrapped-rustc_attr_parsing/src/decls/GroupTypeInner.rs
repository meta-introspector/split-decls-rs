macro_rules! deps {
    () => {
        GroupTypeInnerAccept!();
        Stage!();
        FinalizeFn!();
    };
}

macro_rules! GroupTypeInner {
    () => {
        deps!();
        pub (super) struct GroupTypeInner < S : Stage > { pub (super) accepters : BTreeMap < & 'static [Symbol] , Vec < GroupTypeInnerAccept < S > > > , pub (super) finalizers : Vec < FinalizeFn < S > > , }
    };
}

GroupTypeInner!()