macro_rules! deps {
    () => {
        PrattParserMap!();
        Op!();
        Pair!();
        Affix!();
        PrattParser!();
        RuleType!();
    };
}

macro_rules! impl_135 {
    () => {
        deps!();
        impl < R : RuleType > PrattParser < R > { # [doc = " Instantiate a new `PrattParser`."] pub fn new () -> Self { Self { prec : PREC_STEP , ops : BTreeMap :: new () , has_prefix : false , has_postfix : false , has_infix : false , } } # [doc = " Add `op` to `PrattParser`."] pub fn op (mut self , op : Op < R >) -> Self { self . prec += PREC_STEP ; let mut iter = Some (op) ; while let Some (Op { rule , affix , next }) = iter . take () { match affix { Affix :: Prefix => self . has_prefix = true , Affix :: Postfix => self . has_postfix = true , Affix :: Infix (_) => self . has_infix = true , } self . ops . insert (rule , (affix , self . prec)) ; iter = next . map (| op | * op) ; } self } # [doc = " Maps primary expressions with a closure `primary`."] pub fn map_primary < 'pratt , 'a , 'i , X , T > (& 'pratt self , primary : X ,) -> PrattParserMap < 'pratt , 'a , 'i , R , X , T > where X : FnMut (Pair < 'i , R >) -> T , R : 'pratt , { PrattParserMap { pratt : self , primary , prefix : None , postfix : None , infix : None , phantom : PhantomData , } } }
    };
}

impl_135!();