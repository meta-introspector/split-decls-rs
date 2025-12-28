macro_rules! WalkTreePrefixProducer {
    () => {
        # [derive (Debug)] struct WalkTreePrefixProducer < 'b , S , B > { to_explore : Vec < S > , seen : Vec < S > , children_of : & 'b B , }
    };
}

WalkTreePrefixProducer!()