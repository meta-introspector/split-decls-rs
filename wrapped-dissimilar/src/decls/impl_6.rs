macro_rules! deps {
    () => {
        RangeBounds!();
        Range!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl < 'a > Range < 'a > { pub fn empty () -> Self { Range { doc : & [] , offset : 0 , len : 0 , } } pub fn new (doc : & 'a [char] , bounds : impl RangeBounds) -> Self { let (offset , len) = bounds . index (doc . len ()) ; Range { doc , offset , len } } pub fn is_empty (& self) -> bool { self . len == 0 } pub fn len_bytes (& self) -> usize { self . chars () . map (char :: len_utf8) . sum () } pub fn substring (& self , bounds : impl RangeBounds) -> Self { let (offset , len) = bounds . index (self . len) ; Range { doc : self . doc , offset : self . offset + offset , len , } } pub fn get (& self , bounds : impl RangeBounds) -> Option < Self > { let (offset , len) = bounds . try_index (self . len) ? ; Some (Range { doc : self . doc , offset : self . offset + offset , len , }) } pub fn split_at (& self , mid : usize) -> (Self , Self) { (self . substring (.. mid) , self . substring (mid ..)) } pub fn chars (& self ,) -> impl Iterator < Item = char > + DoubleEndedIterator + ExactSizeIterator + 'a { slice (* self) . iter () . copied () } pub fn starts_with (& self , prefix : impl AsRef < [char] >) -> bool { slice (* self) . starts_with (prefix . as_ref ()) } pub fn ends_with (& self , suffix : impl AsRef < [char] >) -> bool { slice (* self) . ends_with (suffix . as_ref ()) } pub fn find (& self , needle : impl AsRef < [char] >) -> Option < usize > { find (slice (* self) , needle . as_ref ()) } }
    };
}

impl_6!();