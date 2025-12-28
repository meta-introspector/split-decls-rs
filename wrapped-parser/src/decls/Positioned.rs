macro_rules! deps {
    () => {
        Pos!();
    };
}

macro_rules! Positioned {
    () => {
        deps!();
        # [doc = " An AST node that stores its original position."] # [derive (Debug , Clone , Copy , Default , Serialize , Deserialize)] pub struct Positioned < T : ? Sized > { # [doc = " The position of the node."] pub pos : Pos , # [doc = " The node itself."] pub node : T , }
    };
}

Positioned!()