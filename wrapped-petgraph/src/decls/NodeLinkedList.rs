macro_rules! deps {
    () => {
        FasNodeIndex!();
        FasNode!();
        FasNodeContainer!();
    };
}

macro_rules! NodeLinkedList {
    () => {
        deps!();
        type NodeLinkedList = LinkedList < FasNode , FasNodeContainer , FasNodeIndex > ;
    };
}

NodeLinkedList!();