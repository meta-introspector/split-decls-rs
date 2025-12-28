macro_rules! deps {
    () => {
        Value!();
        Item!();
        Id!();
    };
}

macro_rules! Task {
    () => {
        deps!();
        # [doc = " The value associated with a spot in the hierarchy."] # [derive (Clone , Default , Debug , Hash)] pub struct Task { # [doc = " The name of the `Item` or task."] pub name : String , # [doc = " The stable identifier of this task."] # [doc = " Useful for selecting specific tasks out of a set of them."] pub id : Id , # [doc = " The progress itself, unless this value belongs to an `Item` serving as organizational unit."] pub progress : Option < Value > , }
    };
}

Task!()