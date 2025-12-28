macro_rules! Range {
    () => {
        # [doc = " An inclusive range of codepoints from a generated file (hence the static"] # [doc = " lifetime)."] type Range = & 'static [(char , char)] ;
    };
}

Range!();