macro_rules! deps {
    () => {
        DrainProducer!();
        Producer!();
        IntoIter!();
        SliceDrain!();
    };
}

macro_rules! impl_1358 {
    () => {
        deps!();
        impl < 'data , T : 'data + Send > Producer for DrainProducer < 'data , T > { type Item = T ; type IntoIter = SliceDrain < 'data , T > ; fn into_iter (mut self) -> Self :: IntoIter { let slice = mem :: take (& mut self . slice) ; SliceDrain { iter : slice . iter_mut () , } } fn split_at (mut self , index : usize) -> (Self , Self) { let slice = mem :: take (& mut self . slice) ; let (left , right) = slice . split_at_mut (index) ; unsafe { (DrainProducer :: new (left) , DrainProducer :: new (right)) } } }
    };
}

impl_1358!()