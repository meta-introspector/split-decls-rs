macro_rules! deps {
    () => {
        Pending!();
    };
}

macro_rules! impl_182 {
    () => {
        deps!();
        impl < A : Stream , B : Stream > Stream for Zip < A , B > { type Item = (A :: Item , B :: Item) ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let this = self . project () ; if this . item_slot . is_none () { match this . first . poll_next (cx) { Poll :: Pending => return Poll :: Pending , Poll :: Ready (None) => return Poll :: Ready (None) , Poll :: Ready (Some (item)) => * this . item_slot = Some (item) , } } let second_item = ready ! (this . second . poll_next (cx)) ; let first_item = this . item_slot . take () . unwrap () ; Poll :: Ready (second_item . map (| second_item | (first_item , second_item))) } }
    };
}

impl_182!()