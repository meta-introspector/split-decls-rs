macro_rules! deps {
    () => {
        Adjacency!();
        Level!();
        SiblingLocation!();
    };
}

macro_rules! impl_149 {
    () => {
        deps!();
        impl Adjacency { # [doc = " Return the level at which this sibling is located in the hierarchy."] pub fn level (& self) -> Level { use SiblingLocation :: * ; match self { Adjacency (NotFound , NotFound , NotFound , NotFound , NotFound , NotFound) => 0 , Adjacency (_a , NotFound , NotFound , NotFound , NotFound , NotFound) => 1 , Adjacency (_a , _b , NotFound , NotFound , NotFound , NotFound) => 2 , Adjacency (_a , _b , _c , NotFound , NotFound , NotFound) => 3 , Adjacency (_a , _b , _c , _d , NotFound , NotFound) => 4 , Adjacency (_a , _b , _c , _d , _e , NotFound) => 5 , Adjacency (_a , _b , _c , _d , _e , _f) => 6 , } } # [doc = " Get a reference to the sibling location at `level`."] pub fn get (& self , level : Level) -> Option < & SiblingLocation > { Some (match level { 1 => & self . 0 , 2 => & self . 1 , 3 => & self . 2 , 4 => & self . 3 , 5 => & self . 4 , 6 => & self . 5 , _ => return None , }) } # [doc = " Get a mutable reference to the sibling location at `level`."] pub fn get_mut (& mut self , level : Level) -> Option < & mut SiblingLocation > { Some (match level { 1 => & mut self . 0 , 2 => & mut self . 1 , 3 => & mut self . 2 , 4 => & mut self . 3 , 5 => & mut self . 4 , 6 => & mut self . 5 , _ => return None , }) } }
    };
}

impl_149!()