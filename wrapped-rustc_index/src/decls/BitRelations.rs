macro_rules! BitRelations {
    () => {
        pub trait BitRelations < Rhs > { fn union (& mut self , other : & Rhs) -> bool ; fn subtract (& mut self , other : & Rhs) -> bool ; fn intersect (& mut self , other : & Rhs) -> bool ; }
    };
}

BitRelations!()