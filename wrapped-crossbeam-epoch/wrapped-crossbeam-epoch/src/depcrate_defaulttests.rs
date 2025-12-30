// Generated macro for tests (module)
macro_rules! Depcrate_defaulttests {
() => {
// Module: crate::default
// Provides: {"tests"}
// Dependencies: {}
# [cfg (all (test , not (crossbeam_loom)))] mod tests { use crossbeam_utils :: thread ; # [test] fn pin_while_exiting () { struct Foo ; impl Drop for Foo { fn drop (& mut self) { super :: pin () ; } } std :: thread_local ! { static FOO : Foo = const { Foo } ; } thread :: scope (| scope | { scope . spawn (| _ | { FOO . with (| _ | ()) ; super :: pin () ; }) ; }) . unwrap () ; } }
};
}
