macro_rules! deps {
    () => {
        LifetimeContext!();
        NameRefContext!();
        NameContext!();
    };
}

macro_rules! CompletionAnalysis {
    () => {
        deps!();
        # [doc = " The identifier we are currently completing."] # [derive (Debug)] pub (crate) enum CompletionAnalysis < 'db > { Name (NameContext) , NameRef (NameRefContext < 'db >) , Lifetime (LifetimeContext) , # [doc = " The string the cursor is currently inside"] String { # [doc = " original token"] original : ast :: String , # [doc = " fake token"] expanded : Option < ast :: String > , } , # [doc = " Set if we are currently completing in an unexpanded attribute, this usually implies a builtin attribute like `allow($0)`"] UnexpandedAttrTT { colon_prefix : bool , fake_attribute_under_caret : Option < ast :: Attr > , extern_crate : Option < ast :: ExternCrate > , } , }
    };
}

CompletionAnalysis!()