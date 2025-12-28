macro_rules! deps {
    () => {
        Csr!();
    };
}

macro_rules! impl_518 {
    () => {
        deps!();
        impl < N : Clone , E : Clone , Ty , Ix : Clone > Clone for Csr < N , E , Ty , Ix > { fn clone (& self) -> Self { Csr { column : self . column . clone () , edges : self . edges . clone () , row : self . row . clone () , node_weights : self . node_weights . clone () , edge_count : self . edge_count , ty : self . ty , } } }
    };
}

impl_518!()