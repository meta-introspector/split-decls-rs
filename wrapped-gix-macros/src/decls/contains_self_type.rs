macro_rules! contains_self_type {
    () => {
        fn contains_self_type (input : & Type) -> bool { match input { Type :: Array (TypeArray { elem , len , .. }) => { contains_self_type_expr (len) || contains_self_type (elem) } Type :: Group (TypeGroup { elem , .. }) => contains_self_type (elem) , Type :: Paren (TypeParen { elem , .. }) => contains_self_type (elem) , Type :: Ptr (TypePtr { elem , .. }) => contains_self_type (elem) , Type :: Reference (TypeReference { elem , .. }) => contains_self_type (elem) , Type :: Slice (TypeSlice { elem , .. }) => contains_self_type (elem) , Type :: Tuple (TypeTuple { elems , .. }) => elems . iter () . any (contains_self_type) , Type :: Path (TypePath { qself : Some (_) , .. }) => true , Type :: Path (TypePath { path , .. }) => contains_self_type_path (path) , _ => false , } }
    };
}

contains_self_type!();