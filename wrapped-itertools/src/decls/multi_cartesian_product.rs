macro_rules! deps {
    () => {
        MultiProduct!();
        MultiProductIter!();
        MultiProductInner!();
    };
}

macro_rules! multi_cartesian_product {
    () => {
        deps!();
        # [doc = " Create a new cartesian product iterator over an arbitrary number"] # [doc = " of iterators of the same type."] # [doc = ""] # [doc = " Iterator element is of type `Vec<H::Item::Item>`."] pub fn multi_cartesian_product < H > (iters : H) -> MultiProduct < < H :: Item as IntoIterator > :: IntoIter > where H : Iterator , H :: Item : IntoIterator , < H :: Item as IntoIterator > :: IntoIter : Clone , < H :: Item as IntoIterator > :: Item : Clone , { let inner = MultiProductInner { iters : iters . map (| i | MultiProductIter :: new (i . into_iter ())) . collect () , cur : NotYetPopulated , } ; MultiProduct (ProductInProgress (inner)) }
    };
}

multi_cartesian_product!()