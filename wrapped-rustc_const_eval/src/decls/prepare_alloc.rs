macro_rules! deps {
    () => {
        Machine!();
        InternError!();
        MemoryKind!();
    };
}

macro_rules! prepare_alloc {
    () => {
        deps!();
        fn prepare_alloc < 'tcx , Prov : Provenance , Extra , Bytes : AllocBytes > (tcx : TyCtxt < 'tcx > , kind : MemoryKind < const_eval :: MemoryKind > , alloc : & mut Allocation < Prov , Extra , Bytes > , mutability : Mutability ,) -> Result < () , InternError > { match kind { MemoryKind :: Machine (const_eval :: MemoryKind :: Heap { was_made_global }) => { if ! was_made_global { tcx . dcx () . delayed_bug ("non-global heap allocation in const value") ; return Err (InternError :: ConstAllocNotGlobal) ; } } MemoryKind :: Stack | MemoryKind :: CallerLocation => { } } if ! alloc . provenance_merge_bytes (& tcx) { tcx . dcx () . delayed_bug ("partial pointer in const value") ; return Err (InternError :: PartialPointer) ; } match mutability { Mutability :: Not => { alloc . mutability = Mutability :: Not ; } Mutability :: Mut => { assert_eq ! (alloc . mutability , Mutability :: Mut) ; } } Ok (()) }
    };
}

prepare_alloc!();