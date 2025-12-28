macro_rules! deps {
    () => {
        Scope!();
    };
}

macro_rules! ScopedThreadBuilder {
    () => {
        deps!();
        # [doc = " Configures the properties of a new thread."] # [doc = ""] # [doc = " The two configurable properties are:"] # [doc = ""] # [doc = " - [`name`]: Specifies an [associated name for the thread][naming-threads]."] # [doc = " - [`stack_size`]: Specifies the [desired stack size for the thread][stack-size]."] # [doc = ""] # [doc = " The [`spawn`] method will take ownership of the builder and return an [`io::Result`] of the"] # [doc = " thread handle with the given configuration."] # [doc = ""] # [doc = " The [`Scope::spawn`] method uses a builder with default configuration and unwraps its return"] # [doc = " value. You may want to use this builder when you want to recover from a failure to launch a"] # [doc = " thread."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use crossbeam_utils::thread;"] # [doc = ""] # [doc = " thread::scope(|s| {"] # [doc = "     s.builder()"] # [doc = "         .spawn(|_| println!(\"Running a child thread\"))"] # [doc = "         .unwrap();"] # [doc = " }).unwrap();"] # [doc = " ```"] # [doc = ""] # [doc = " [`name`]: ScopedThreadBuilder::name"] # [doc = " [`stack_size`]: ScopedThreadBuilder::stack_size"] # [doc = " [`spawn`]: ScopedThreadBuilder::spawn"] # [doc = " [`io::Result`]: std::io::Result"] # [doc = " [naming-threads]: std::thread#naming-threads"] # [doc = " [stack-size]: std::thread#stack-size"] # [must_use = "must eventually spawn the thread"] # [derive (Debug)] pub struct ScopedThreadBuilder < 'scope , 'env > { scope : & 'scope Scope < 'env > , builder : thread :: Builder , }
    };
}

ScopedThreadBuilder!()