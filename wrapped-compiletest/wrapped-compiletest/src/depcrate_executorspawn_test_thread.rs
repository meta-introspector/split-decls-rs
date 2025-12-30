// Generated macro for spawn_test_thread (function)
macro_rules! Depcrate_executorspawn_test_thread {
() => {
// Module: crate::executor
// Provides: {"spawn_test_thread"}
// Dependencies: {}
# [doc = " Spawns a thread to run a single test, and returns the thread's join handle."] # [doc = ""] # [doc = " Returns `None` if the test was ignored, so no thread was spawned."] fn spawn_test_thread (id : TestId , test : & CollectedTest , completion_tx : mpsc :: Sender < TestCompletion > ,) -> Option < thread :: JoinHandle < () > > { if test . desc . ignore && ! test . config . run_ignored { completion_tx . send (TestCompletion { id , outcome : TestOutcome :: Ignored , stdout : None }) . unwrap () ; return None ; } let runnable_test = RunnableTest :: new (test) ; let should_panic = test . desc . should_panic ; let run_test = move | | run_test_inner (id , should_panic , runnable_test , completion_tx) ; let thread_builder = thread :: Builder :: new () . name (test . desc . name . clone ()) ; let join_handle = thread_builder . spawn (run_test) . unwrap () ; Some (join_handle) }
};
}
