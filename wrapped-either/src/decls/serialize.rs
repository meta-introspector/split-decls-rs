macro_rules! deps {
    () => {
        Either!();
    };
}

macro_rules! serialize {
    () => {
        deps!();
        pub fn serialize < L , R , S > (this : & Option < super :: Either < L , R > > , serializer : S ,) -> Result < S :: Ok , S :: Error > where S : Serializer , L : Serialize , R : Serialize , { let untagged = match this { Some (super :: Either :: Left (left)) => Some (Either :: Left (left)) , Some (super :: Either :: Right (right)) => Some (Either :: Right (right)) , None => None , } ; untagged . serialize (serializer) }
    };
}

serialize!()