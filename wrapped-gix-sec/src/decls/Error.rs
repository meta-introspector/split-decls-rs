macro_rules! Error {
    () => {
        # [doc = " An error to use if an operation cannot proceed due to insufficient permissions."] # [doc = ""] # [doc = " It's up to the implementation to decide which permission is required for an operation, and which one"] # [doc = " causes errors."] # [derive (Debug)] pub struct Error < R : std :: fmt :: Debug > { # [doc = " The resource which cannot be used."] pub resource : R , }
    };
}

Error!();