macro_rules! deps {
    () => {
        Action!();
        Context!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        # [doc = " Initialization"] impl Action { # [doc = " Create a `Get` action with context containing the given URL."] # [doc = " Note that this creates an `Action` suitable for the credential helper cascade only."] pub fn get_for_url (url : impl Into < BString >) -> Action { Action :: Get (Context { url : Some (url . into ()) , .. Default :: default () }) } }
    };
}

impl_7!();