macro_rules! deps {
    () => {
        NodeLinkedList!();
    };
}

macro_rules! Buckets {
    () => {
        deps!();
        # [derive (Debug)] struct Buckets { sinks_or_isolated : NodeLinkedList , sources : NodeLinkedList , # [doc = " Bidirectional nodes with positive-or-0 delta degree"] bidirectional_pve_dd : Vec < NodeLinkedList > , # [doc = " Bidirectional nodes with negative delta degree (index 0 is -1 dd, 1 is -2 etc)"] bidirectional_nve_dd : Vec < NodeLinkedList > , }
    };
}

Buckets!()