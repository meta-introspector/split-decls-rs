macro_rules! deps {
    () => {
        Indices!();
    };
}

macro_rules! impl_494 {
    () => {
        deps!();
        # [doc = " Creates an empty iterator."] impl Default for Indices < '_ > { fn default () -> Self { static EMPTY : [usize ; 0] = [] ; Indices { iter : EMPTY [..] . iter () . cloned () , len : 0 , } } }
    };
}

impl_494!()