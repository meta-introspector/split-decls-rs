macro_rules! ErrorKind {
    () => {
        # [derive (Clone , Debug , Eq , Hash , PartialEq)] pub (crate) enum ErrorKind { # [doc = " Invalid character in the [`Uuid`] string."] # [doc = ""] # [doc = " [`Uuid`]: ../struct.Uuid.html"] Char { character : char , index : usize } , # [doc = " A simple [`Uuid`] didn't contain 32 characters."] # [doc = ""] # [doc = " [`Uuid`]: ../struct.Uuid.html"] SimpleLength { len : usize } , # [doc = " A hyphenated [`Uuid`] didn't contain 5 groups"] # [doc = ""] # [doc = " [`Uuid`]: ../struct.Uuid.html"] GroupCount { count : usize } , # [doc = " A hyphenated [`Uuid`] had a group that wasn't the right length"] # [doc = ""] # [doc = " [`Uuid`]: ../struct.Uuid.html"] GroupLength { group : usize , len : usize , index : usize , } , }
    };
}

ErrorKind!()