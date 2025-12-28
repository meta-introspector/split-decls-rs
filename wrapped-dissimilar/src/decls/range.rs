macro_rules! deps {
    () => {
        Range!();
    };
}

macro_rules! range {
    () => {
        deps!();
        fn range < 'a > (doc : & 'a [char] , offset : & mut usize , text : & str) -> Range < 'a > { let len = text . chars () . count () ; let range = Range { doc , offset : * offset , len , } ; * offset += len ; range }
    };
}

range!()