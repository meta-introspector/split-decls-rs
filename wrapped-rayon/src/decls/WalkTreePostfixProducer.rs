macro_rules! WalkTreePostfixProducer {
    () => {
        # [derive (Debug)] struct WalkTreePostfixProducer < 'b , S , B > { to_explore : Vec < S > , seen : Vec < S > , children_of : & 'b B , }
    };
}

WalkTreePostfixProducer!()