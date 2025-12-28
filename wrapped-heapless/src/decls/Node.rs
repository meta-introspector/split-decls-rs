macro_rules! deps {
    () => {
        SortedLinkedList!();
    };
}

macro_rules! Node {
    () => {
        deps!();
        # [doc = " A node in the [`SortedLinkedList`]."] # [cfg_attr (feature = "zeroize" , derive (Zeroize))] pub struct Node < T , Idx > { val : MaybeUninit < T > , next : Idx , }
    };
}

Node!()