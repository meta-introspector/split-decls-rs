macro_rules! deps {
    () => {
        Field!();
    };
}

macro_rules! from_field {
    () => {
        deps!();
        fn from_field < 'a , 'b > (fields : & 'a [Field < 'b >]) -> Option < & 'a Field < 'b > > { for field in fields { if field . attrs . from . is_some () { return Some (field) ; } } None }
    };
}

from_field!()