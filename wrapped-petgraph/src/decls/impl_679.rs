macro_rules! deps {
    () => {
        Graph!();
    };
}

macro_rules! impl_679 {
    () => {
        deps!();
        # [doc = " The resulting cloned graph has the same graph indices as `self`."] impl < N , E , Ty , Ix > Clone for Graph < N , E , Ty , Ix > where N : Clone , E : Clone , Ix : Copy , { fn clone (& self) -> Self { Graph { nodes : self . nodes . clone () , edges : self . edges . clone () , ty : self . ty , } } fn clone_from (& mut self , rhs : & Self) { self . nodes . clone_from (& rhs . nodes) ; self . edges . clone_from (& rhs . edges) ; self . ty = rhs . ty ; } }
    };
}

impl_679!();