macro_rules! deps {
    () => {
        Event!();
        JoinHandle!();
    };
}

macro_rules! impl_84 {
    () => {
        deps!();
        impl JoinHandle { # [doc = " `detach()` and `forget()` to remove any effects associated with this handle."] pub fn detach (mut self) { self . disconnect () ; self . forget () ; } # [doc = " Remove the handles capability to instruct the render thread to stop, but it will still wait for it"] # [doc = " if dropped."] # [doc = " Use `forget()` if it should not wait for the render thread anymore."] pub fn disconnect (& mut self) { self . disconnected = true ; } # [doc = " Remove the handles capability to `join()` by forgetting the threads handle"] pub fn forget (& mut self) { self . inner . take () ; } # [doc = " Wait for the thread to shutdown naturally, for example because there is no more progress to display"] pub fn wait (mut self) { self . inner . take () . and_then (| h | h . join () . ok ()) ; } # [doc = " Send the shutdown signal right after one last redraw"] pub fn shutdown (& mut self) { if ! self . disconnected { self . connection . send (Event :: Tick) . ok () ; self . connection . send (Event :: Quit) . ok () ; } } # [doc = " Send the signal to shutdown and wait for the thread to be shutdown."] pub fn shutdown_and_wait (mut self) { self . shutdown () ; self . wait () ; } }
    };
}

impl_84!()