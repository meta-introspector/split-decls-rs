macro_rules! deps {
    () => {
        Product!();
    };
}

macro_rules! cartesian_product {
    () => {
        deps!();
        # [doc = " Create a new cartesian product iterator"] # [doc = ""] # [doc = " Iterator element type is `(I::Item, J::Item)`."] pub fn cartesian_product < I , J > (i : I , j : J) -> Product < I , J > where I : Iterator , J : Clone + Iterator , I :: Item : Clone , { Product { a_cur : None , a : i , b : j . clone () , b_orig : j , } }
    };
}

cartesian_product!()