macro_rules! Interrupt {
    () => {
        # [doc = " The variants represented here allow the user to control when the GUI can be shutdown."] # [derive (Debug , Clone , Copy)] pub enum Interrupt { # [doc = " Immediately exit the GUI event loop when there is an interrupt request."] # [doc = ""] # [doc = " This is the default when the event loop is entered."] Instantly , # [doc = " Instead of exiting the event loop instantly, wait until the next Interrupt::Instantly"] # [doc = " event is coming in."] Deferred , }
    };
}

Interrupt!();