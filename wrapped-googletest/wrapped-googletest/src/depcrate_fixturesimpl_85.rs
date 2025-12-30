// Generated macro for impl_85 (impl)
macro_rules! Depcrate_fixturesimpl_85 {
() => {
// Module: crate::fixtures
// Provides: {"impl_85"}
// Dependencies: {}
impl < F : StaticFixture + 'static > Fixture for & 'static F { fn set_up () -> crate :: Result < Self > { static ONCE_FIXTURE_REPO : OnceLock < Mutex < HashMap < TypeId , & 'static (dyn Any + Sync + Send) > > , > = OnceLock :: new () ; let mut map = ONCE_FIXTURE_REPO . get_or_init (| | Mutex :: new (HashMap :: new ())) . lock () ? ; let any = map . entry (TypeId :: of :: < F > ()) . or_insert_with (| | Box :: leak (Box :: new (F :: set_up_once ()))) ; match any . downcast_ref :: < crate :: Result < F > > () { Some (Ok (ref fixture)) => Ok (fixture) , Some (Err (e)) => Err (e . clone ()) , None => panic ! ("Downcast failed. This is a bug in GoogleTest Rust") , } } fn tear_down (self) -> crate :: Result < () > { Ok (()) } }
};
}
