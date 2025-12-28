macro_rules! precompose_path {
    () => {
        # [doc = " Return the precomposed version of `path`, or `path` itself if it contained illformed unicode,"] # [doc = " or if the unicode version didn't contains decomposed unicode."] # [doc = " Otherwise, similar to [`precompose()`]"] pub fn precompose_path (path : Cow < '_ , Path >) -> Cow < '_ , Path > { match path . to_str () { None => path , Some (maybe_decomposed) => match precompose (maybe_decomposed . into ()) { Cow :: Borrowed (_) => path , Cow :: Owned (precomposed) => Cow :: Owned (precomposed . into ()) , } , } }
    };
}

precompose_path!();