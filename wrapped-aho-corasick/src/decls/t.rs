macro_rules! deps {
    () => {
        SearchTest!();
    };
}

macro_rules! t {
    () => {
        deps!();
        # [doc = " Short-hand constructor for SearchTest. We use it a lot below."] macro_rules ! t { ($ name : ident , $ patterns : expr , $ haystack : expr , $ matches : expr) => { SearchTest { name : stringify ! ($ name) , patterns : $ patterns , haystack : $ haystack , matches : $ matches , } } ; }
    };
}

t!()