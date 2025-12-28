macro_rules! Action {
    () => {
        # [doc = " The action passed to the credential helper implementation in [`main()`][crate::program::main()]."] # [derive (Debug , Copy , Clone)] pub enum Action { # [doc = " Get credentials for a url."] Get , # [doc = " Store credentials provided in the given context."] Store , # [doc = " Erase credentials identified by the given context."] Erase , }
    };
}

Action!()