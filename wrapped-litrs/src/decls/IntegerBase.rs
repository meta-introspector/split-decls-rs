macro_rules! IntegerBase {
    () => {
        # [doc = " The bases in which an integer can be specified."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub enum IntegerBase { Binary , Octal , Decimal , Hexadecimal , }
    };
}

IntegerBase!();