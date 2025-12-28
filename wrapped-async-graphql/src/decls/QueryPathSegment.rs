macro_rules! QueryPathSegment {
    () => {
        # [doc = " A segment in the path to the current query."] # [doc = ""] # [doc = " This is a borrowed form of [`PathSegment`](enum.PathSegment.html) used"] # [doc = " during execution instead of passed back when errors occur."] # [derive (Debug , Clone , Copy , Serialize)] # [serde (untagged)] pub enum QueryPathSegment < 'a > { # [doc = " We are currently resolving an element in a list."] Index (usize) , # [doc = " We are currently resolving a field in an object."] Name (& 'a str) , }
    };
}

QueryPathSegment!()