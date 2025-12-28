macro_rules! UnionFind {
    () => {
        # [doc = " `UnionFind<K>` is a disjoint-set data structure. It tracks set membership of *n* elements"] # [doc = " indexed from *0* to *n - 1*. The scalar type is `K` which must be an unsigned integer type."] # [doc = ""] # [doc = " <http://en.wikipedia.org/wiki/Disjoint-set_data_structure>"] # [doc = ""] # [doc = " Too awesome not to quote:"] # [doc = ""] # [doc = " “The amortized time per operation is **O(α(n))** where **α(n)** is the"] # [doc = " inverse of **f(x) = A(x, x)** with **A** being the extremely fast-growing Ackermann function.”"] # [derive (Debug , Clone)] pub struct UnionFind < K > { parent : Vec < K > , rank : Vec < u8 > , }
    };
}

UnionFind!()