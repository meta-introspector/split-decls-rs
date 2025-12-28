macro_rules! deps {
    () => {
        UseStatement!();
    };
}

macro_rules! ClassifiedUseStatements {
    () => {
        deps!();
        # [derive (Debug)] pub struct ClassifiedUseStatements (pub Vec < UseStatement > , pub HashMap < String , Vec < String > >) ;
    };
}

ClassifiedUseStatements!()