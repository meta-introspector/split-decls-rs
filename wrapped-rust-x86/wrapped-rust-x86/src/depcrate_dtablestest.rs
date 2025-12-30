// Generated macro for test (module)
macro_rules! Depcrate_dtablestest {
() => {
// Module: crate::dtables
// Provides: {"test"}
// Dependencies: {}
# [cfg (all (test , feature = "utest"))] mod test { use super :: * ; # [test] fn check_sgdt () { let mut gdtr : super :: DescriptorTablePointer < u64 > = Default :: default () ; gdtr . limit = 0xdead ; gdtr . base = 0xbadc0de as * mut u64 ; unsafe { sgdt (& mut gdtr) ; } let base = gdtr . base ; let limit = gdtr . limit ; assert_ne ! (base , core :: ptr :: null_mut ()) ; assert_ne ! (limit , 0xdead) ; assert_ne ! (base as u64 , 0xbadc0de) ; } # [test] fn check_sidt () { let mut gdtr : super :: DescriptorTablePointer < u64 > = Default :: default () ; gdtr . limit = 0xdead ; gdtr . base = 0xbadc0de as * mut u64 ; unsafe { sidt (& mut gdtr) ; } let base = gdtr . base ; let limit = gdtr . limit ; assert_ne ! (base , core :: ptr :: null_mut ()) ; assert_ne ! (limit , 0xdead) ; assert_ne ! (base as u64 , 0xbadc0de) ; } }
};
}
