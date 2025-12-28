macro_rules! no_expansion {
    () => {
        # [doc = " Quickly checks the given replacement string for whether interpolation"] # [doc = " should be done on it. It returns `None` if a `$` was found anywhere in the"] # [doc = " given string, which suggests interpolation needs to be done. But if there's"] # [doc = " no `$` anywhere, then interpolation definitely does not need to be done. In"] # [doc = " that case, the given string is returned as a borrowed `Cow`."] # [doc = ""] # [doc = " This is meant to be used to implement the [`Replacer::no_expansion`] method"] # [doc = " in its various trait impls."] fn no_expansion < T : AsRef < str > > (replacement : & T) -> Option < Cow < '_ , str > > { let replacement = replacement . as_ref () ; match crate :: find_byte :: find_byte (b'$' , replacement . as_bytes ()) { Some (_) => None , None => Some (Cow :: Borrowed (replacement)) , } }
    };
}

no_expansion!()