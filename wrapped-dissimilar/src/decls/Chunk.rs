macro_rules! Chunk {
    () => {
        # [derive (Copy , Clone , PartialEq , Eq)] pub enum Chunk < 'a > { Equal (& 'a str) , Delete (& 'a str) , Insert (& 'a str) , }
    };
}

Chunk!();