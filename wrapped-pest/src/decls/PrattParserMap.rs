macro_rules! deps {
    () => {
        InfixFn!();
        RuleType!();
        PostfixFn!();
        PrefixFn!();
        PrattParser!();
        Pair!();
    };
}

macro_rules! PrattParserMap {
    () => {
        deps!();
        # [doc = " Product of calling [`map_primary`] on [`PrattParser`], defines how expressions should"] # [doc = " be mapped."] # [doc = ""] # [doc = " [`map_primary`]: struct.PrattParser.html#method.map_primary"] # [doc = " [`PrattParser`]: struct.PrattParser.html"] pub struct PrattParserMap < 'pratt , 'a , 'i , R , F , T > where R : RuleType , F : FnMut (Pair < 'i , R >) -> T , { pratt : & 'pratt PrattParser < R > , primary : F , prefix : Option < PrefixFn < 'a , 'i , R , T > > , postfix : Option < PostfixFn < 'a , 'i , R , T > > , infix : Option < InfixFn < 'a , 'i , R , T > > , phantom : PhantomData < T > , }
    };
}

PrattParserMap!()