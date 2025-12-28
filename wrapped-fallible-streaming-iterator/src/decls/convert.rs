macro_rules! deps {
    () => {
        FallibleStreamingIterator!();
        Convert!();
    };
}

macro_rules! convert {
    () => {
        deps!();
        # [doc = " Converts a normal `Iterator` over `Results` of references into a"] # [doc = " `FallibleStreamingIterator`."] pub fn convert < 'a , I , T , E > (it : I) -> Convert < 'a , I , T > where I : Iterator < Item = Result < & 'a T , E > > , { Convert { it : it , item : None } }
    };
}

convert!()