macro_rules! PathParser {
    () => {
        # [derive (Clone , Debug)] pub struct PathParser < 'a > (pub Cow < 'a , Path >) ;
    };
}

PathParser!()