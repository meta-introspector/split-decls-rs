// Generated macro for tests (module)
macro_rules! Depcrate_main_thread_boundtests {
() => {
// Module: crate::main_thread_bound
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use core :: cell :: Cell ; static_assertions :: assert_impl_all ! (MainThreadBound < MainThreadMarker >: Send , Sync) ; static_assertions :: assert_impl_all ! (MainThreadBound <* const () >: Send , Sync) ; # [test] fn always_send_sync () { fn assert_send_sync < T : Send + Sync > () { } fn foo < T > () { assert_send_sync :: < MainThreadBound < T > > () ; } foo :: < () > () ; } # [test] fn test_main_thread_bound_into_inner () { let mtm = unsafe { MainThreadMarker :: new_unchecked () } ; struct Foo < 'a > { is_dropped : & 'a Cell < bool > , } impl Drop for Foo < '_ > { fn drop (& mut self) { self . is_dropped . set (true) ; } } let is_dropped = Cell :: new (false) ; let foo = Foo { is_dropped : & is_dropped , } ; let foo = MainThreadBound :: new (foo , mtm) ; assert ! (! is_dropped . get ()) ; let foo = foo . into_inner (mtm) ; assert ! (! is_dropped . get ()) ; drop (foo) ; assert ! (is_dropped . get ()) ; } }
};
}
