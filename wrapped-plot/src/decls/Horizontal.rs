macro_rules! Horizontal {
    () => {
        # [doc = " Horizontal position of the key"] # [derive (Clone , Copy)] pub enum Horizontal { # [doc = " Center of the figure"] Center , # [doc = " Left border of the figure"] Left , # [doc = " Right border of the figure"] Right , }
    };
}

Horizontal!()