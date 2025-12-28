macro_rules! deps {
    () => {
        ScopedJoinHandle!();
    };
}

macro_rules! impl_164 {
    () => {
        deps!();
        impl < T > ScopedJoinHandle < '_ , T > { # [doc = " Waits for the thread to finish and returns its result."] # [doc = ""] # [doc = " If the child thread panics, an error is returned. Note that if panics are implemented by"] # [doc = " aborting the process, no error is returned; see the notes of [std::panic::catch_unwind]."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " This function may panic on some platforms if a thread attempts to join itself or otherwise"] # [doc = " may create a deadlock with joining threads."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use crossbeam_utils::thread;"] # [doc = ""] # [doc = " thread::scope(|s| {"] # [doc = "     let handle1 = s.spawn(|_| println!(\"I'm a happy thread :)\"));"] # [doc = "     let handle2 = s.spawn(|_| panic!(\"I'm a sad thread :(\"));"] # [doc = ""] # [doc = "     // Join the first thread and verify that it succeeded."] # [doc = "     let res = handle1.join();"] # [doc = "     assert!(res.is_ok());"] # [doc = ""] # [doc = "     // Join the second thread and verify that it panicked."] # [doc = "     let res = handle2.join();"] # [doc = "     assert!(res.is_err());"] # [doc = " }).unwrap();"] # [doc = " ```"] pub fn join (self) -> thread :: Result < T > { let handle = self . handle . lock () . unwrap () . take () . unwrap () ; handle . join () . map (| () | self . result . lock () . unwrap () . take () . unwrap ()) } # [doc = " Returns a handle to the underlying thread."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use crossbeam_utils::thread;"] # [doc = ""] # [doc = " thread::scope(|s| {"] # [doc = "     let handle = s.spawn(|_| println!(\"A child thread is running\"));"] # [doc = "     println!(\"The child thread ID: {:?}\", handle.thread().id());"] # [doc = " }).unwrap();"] # [doc = " ```"] pub fn thread (& self) -> & thread :: Thread { & self . thread } }
    };
}

impl_164!()