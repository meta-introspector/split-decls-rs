macro_rules! Signature {
    () => {
        # [doc = " A Signature is used to indicate authorship of various actions throughout the"] # [doc = " library."] # [doc = ""] # [doc = " Signatures contain a name, email, and timestamp. All fields can be specified"] # [doc = " with `new` while the `now` constructor omits the timestamp. The"] # [doc = " [`Repository::signature`] method can be used to create a default signature"] # [doc = " with name and email values read from the configuration."] # [doc = ""] # [doc = " [`Repository::signature`]: struct.Repository.html#method.signature"] pub struct Signature < 'a > { raw : * mut raw :: git_signature , _marker : marker :: PhantomData < & 'a str > , owned : bool , }
    };
}

Signature!();