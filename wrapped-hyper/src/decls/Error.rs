macro_rules! deps {
    () => {
        ErrorImpl!();
    };
}

macro_rules! Error {
    () => {
        deps!();
        # [doc = " Represents errors that can occur handling HTTP streams."] # [doc = ""] # [doc = " # Formatting"] # [doc = ""] # [doc = " The `Display` implementation of this type will only print the details of"] # [doc = " this level of error, even though it may have been caused by another error"] # [doc = " and contain that error in its source. To print all the relevant"] # [doc = " information, including the source chain, using something like"] # [doc = " `std::error::Report`, or equivalent 3rd party types."] # [doc = ""] # [doc = " The contents of the formatted error message of this specific `Error` type"] # [doc = " is unspecified. **You must not depend on it.** The wording and details may"] # [doc = " change in any version, with the goal of improving error messages."] # [doc = ""] # [doc = " # Source"] # [doc = ""] # [doc = " A `hyper::Error` may be caused by another error. To aid in debugging,"] # [doc = " those are exposed in `Error::source()` as erased types. While it is"] # [doc = " possible to check the exact type of the sources, they **can not be depended"] # [doc = " on**. They may come from private internal dependencies, and are subject to"] # [doc = " change at any moment."] pub struct Error { inner : Box < ErrorImpl > , }
    };
}

Error!()