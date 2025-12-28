macro_rules! deps {
    () => {
        GatherUsedMutsVisitor!();
        MirBorrowckCtxt!();
    };
}

macro_rules! impl_504 {
    () => {
        deps!();
        impl < 'tcx > MirBorrowckCtxt < '_ , '_ , 'tcx > { # [doc = " Walks the MIR adding to the set of `used_mut` locals that will be ignored for the purposes"] # [doc = " of the `unused_mut` lint."] # [doc = ""] # [doc = " `temporary_used_locals` should contain locals that were found to be temporary, mutable and"] # [doc = "  used from borrow checking. This function looks for assignments into these locals from"] # [doc = "  user-declared locals and adds those user-defined locals to the `used_mut` set. This can"] # [doc = "  occur due to a rare case involving upvars in closures."] # [doc = ""] # [doc = " `never_initialized_mut_locals` should contain the set of user-declared mutable locals"] # [doc = "  (not arguments) that have not already been marked as being used."] # [doc = "  This function then looks for assignments from statements or the terminator into the locals"] # [doc = "  from this set and removes them from the set. This leaves only those locals that have not"] # [doc = "  been assigned to - this set is used as a proxy for locals that were not initialized due to"] # [doc = "  unreachable code. These locals are then considered \"used\" to silence the lint for them."] # [doc = "  See #55344 for context."] pub (crate) fn gather_used_muts (& mut self , temporary_used_locals : FxIndexSet < Local > , mut never_initialized_mut_locals : FxIndexSet < Local > ,) { { let mut visitor = GatherUsedMutsVisitor { temporary_used_locals , never_initialized_mut_locals : & mut never_initialized_mut_locals , mbcx : self , } ; visitor . visit_body (visitor . mbcx . body) ; } debug ! ("gather_used_muts: never_initialized_mut_locals={:?}" , never_initialized_mut_locals) ; self . used_mut = self . used_mut . union (& never_initialized_mut_locals) . cloned () . collect () ; } }
    };
}

impl_504!();