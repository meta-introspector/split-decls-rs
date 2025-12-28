macro_rules! FlattenFolder {
    () => {
        struct FlattenFolder < C , R > { base : C , previous : Option < R > , }
    };
}

FlattenFolder!()