macro_rules! deps {
    () => {
        FasNode!();
        FasNodeIndex!();
        FasNodeContainer!();
    };
}

macro_rules! NodeLinkedList {
    () => {
        deps!();
        type NodeLinkedList = LinkedList < FasNode , FasNodeContainer , FasNodeIndex > ;
    };
}

NodeLinkedList!()