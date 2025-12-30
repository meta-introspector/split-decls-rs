// Generated macro for ALLOC (static)
macro_rules! Depcrate_syscall_allocatorALLOC {
() => {
// Module: crate::syscall::allocator
// Provides: {"ALLOC"}
// Dependencies: {}
static ALLOC : Lazy < RawSpinlock , Talck < RawSpinlock , ErrOnOom > > = Lazy :: new (| | { take_static :: take_static ! { static MEM : [MaybeUninit < u8 >; 0x1000] = [MaybeUninit :: uninit () ; 0x1000] ; } let mem = MEM . take () . unwrap () ; let mut talc = Talc :: new (talc :: ErrOnOom) ; unsafe { talc . claim (mem . into ()) . unwrap () ; } Talck :: new (talc) }) ;
};
}
